from flask import request, jsonify
from json import JSONDecodeError

from . import task, app, config, scheduler


# no auth (for the hackathon)
@app.route('/stream', methods=['POST'])
def app_route():
    try:
        task.process_stream(**request.json)
        return jsonify(message='success'), 200
    except (KeyError, JSONDecodeError) as e:
        return jsonify(message=repr(e)), 400
    except BaseException as e:
        return jsonify(message=f'unknown error {repr(e)}'), 500


def start_app():
    scheduler.start()
    app.run(host='0.0.0.0', port=config.APP_PORT)


if __name__ == '__main__':
    start_app()
