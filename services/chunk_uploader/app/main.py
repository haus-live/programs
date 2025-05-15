# ffmpeg -i http://localhost:8080/memfs/617f7575-c9f8-42f2-b706-344c0d997afb.m3u8 -t 15 -c copy test-15.mkv

# config: ipfs creds, solana rpc creds, restreamer host, programid 

# task pool: <stream uuid, stream begin + end timestamps, keypair>

# task meta: stream uuid + keypair
# ffmpeg -> ipfs upload -> metadata update

# task body: ts + duration, 
