import requests
from datetime import datetime 
response = requests.post('http://localhost:8888/stream', json={
    'stream_id': '617f7575-c9f8-42f2-b706-344c0d997afb',
    'asset_pubkey': '8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5',
    'event_begin_timestamp': int(datetime.now().timestamp()) + 5,
    'event_end_timestamp': int(datetime.now().timestamp()) + 5 + 10 + 5
}, headers={'Content-Type': 'application/json'})
print(response.text)


# if __name__ == '__main__':
#     from datetime import datetime
#     from time import sleep
#     scheduler.start()
#     task.process_stream(**{
#         'stream_id': '617f7575-c9f8-42f2-b706-344c0d997afb',
#         'asset_pubkey': '8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5',
#         'event_begin_timestamp': int(datetime.now().timestamp()) + 5,
#         'event_end_timestamp': int(datetime.now().timestamp()) + 5 + 10
#     })
#     while True:
#         sleep(2)
#         print('awake')
#         print(scheduler.get_jobs())

#     # task._process_chunk(chunk_id=0, stream_id='617f7575-c9f8-42f2-b706-344c0d997afb', asset_pubkey='8SjSBampBM2asLdQeJoAZpxJxpcbBEGG5q9ADRCAFxr5')
