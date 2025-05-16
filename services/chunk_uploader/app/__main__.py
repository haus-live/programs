from flask import request, jsonify
from json import JSONDecodeError

from . import task, app, config


@app.route('/', methods=['POST'])
def app_route():
    try:
        request_json = request.json()
        task.process_stream(**request_json)
        return jsonify(message='success'), 200
    except (KeyError, JSONDecodeError) as e:
        return jsonify(message=repr(e)), 400
    except BaseException as e:
        return jsonify(message='unknown error'), 500


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=config.APP_PORT)
