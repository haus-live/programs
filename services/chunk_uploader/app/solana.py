from solana.rpc.api import Client
from solders.pubkey import Pubkey
from solders.keypair import Keypair
from solders.transaction import Transaction
from solders.instruction import Instruction
from solders.message import Message
from solders.account import Account
from solders.system_program import ID
from solders.keypair import Keypair

from solana.rpc.commitment import Confirmed
from solana.rpc.api import Client
from solana.rpc.types import TxOpts

from hashlib import sha256

from borsh_construct import U8, String, CStruct

from app.config import AppConfig

_LOAD_CHUNKS_INSTRUCTION_IDENTIFIER: bytes = sha256(b'haus::load_chunks').digest()[:8]
_LOAD_CHUNKS_ARGS_CSTRUCT = CStruct(
    'uri' / String
)

class Solana:
    def __init__(self, config: AppConfig):
        self._config = config
        self._solana = Client(endpoint=self._config.SOLANA_RPC_URL)
    
    def read_uri_from_asset_account(self, asset_pubkey: str):
        pubkey = Pubkey.from_string(asset_pubkey)
        account_info_json = self._solana.get_account_info_json_parsed(pubkey).value
        return account_info_json.data.to_json['uri']
    
    def update_core_asset_account_uri(self, asset_pubkey: str, new_uri: str):
        haus_program_id = Pubkey.from_string(self._config.HAUS_PROGRAM_ID)

        chunk_updater_pubker = Pubkey.from_string(self._config.CHUNK_UPDATER_PUBLIC_KEY)
        chunk_updater_keypair = Keypair.from_bytes(bytes(self._config.CHUNK_UPDATER_SECRET_KEY))

        # specify accounts
        realtime_asset_pubkey = Pubkey.from_string(asset_pubkey)
        realtime_asset_account = Account.from_bytes(
            self._solana.get_account_info(realtime_asset_pubkey).value
        )
        event_pubkey, _ = Pubkey.find_program_address(
            [b'event', bytes(realtime_asset_pubkey)],
            haus_program_id
        )
        event_account = Account.from_bytes(
            self._solana.get_account_info(event_pubkey).value
        )
        authority_account = Account.from_bytes(
            self._solana.get_account_info(chunk_updater_pubker).value
        )
        system_program_account = Account.from_bytes(
            self._solana.get_account_info(ID).value
        )
        mpl_core_program_account = Account.from_bytes(
            self._solana.get_account_info(
                Pubkey.from_string(self._config.MPL_CORE_ASSET_PROGRAM_ID)
            )
        )

        # build instruction
        load_chunks_args_data = _LOAD_CHUNKS_ARGS_CSTRUCT.build({
            'uri': new_uri
        })
        instruction = Instruction(
            haus_program_id,
            data=_LOAD_CHUNKS_INSTRUCTION_IDENTIFIER + load_chunks_args_data,
            accounts=(
                realtime_asset_account,
                event_account,
                authority_account, # fee paying account
                system_program_account,
                mpl_core_program_account
            )
        )

        # build transaction
        message = Message([instruction], chunk_updater_pubker)
        transaction = Transaction([chunk_updater_keypair], message)

        # send transaction
        self._solana.send_transaction(
            transaction, 
            opts=TxOpts(skip_confirmation=False, preflight_commitment=Confirmed)
        )
