#!/usr/bin/env python3
import flask
import flask_cors

tasks = {}
logs = []

app = flask.Flask(__name__)
flask_cors.CORS(app, resources={r"/*": {"origins": "*"}})

@app.before_request
def block_user_agent():
    user_agent = flask.request.headers.get('User-Agent')
    if user_agent == 'nginx-ssl early hints':
        flask.abort(400)

@app.route("/")
def index():
    return flask.redirect("https://jojojux.de/crs-ui/")


@app.route("/activate/<int:channel>")
def activate(channel: int):
    if tasks.get(channel) is not None:
        return {"error": "Channel is already being listened on.", "error_id": 1}
    tasks[channel] = {"actions": [], "channel": channel}
    return {"success": "Channel was activated."}


@app.route("/loose/<int:channel>")
def loose(channel: int):
    if tasks.get(channel) is None:
        return {"error": "Channel is not being listened on.", "error_id": 2}
    del tasks[channel]
    return {"success": "Channel was deactivated."}


@app.route("/pending/<int:channel>")
def pending_x(channel: int):
    return tasks.get(channel, {}).get(
        "actions", {"error": "Channel is not being listened on.", "error_id": 2}
    )


@app.route("/pop/<int:channel>")
def pop_x(channel: int):
    if tasks.get(channel) is None:
        return {"error": "Channel is not being listened on.", "error_id": 2}
    if len(tasks[channel]["actions"]) == 0:
        return {"next_action": "idle"}
    return {"next_action": tasks[channel]["actions"].pop(0)}


@app.route("/action/<int:channel>")
def queue_x(channel: int):
    if tasks.get(channel) is None:
        return {"error": "Channel is not being listened on.", "error_id": 2}
    tasks[channel]["actions"].append(flask.request.args.to_dict())
    return {
        "action": flask.request.args,
        "success": "The action was added to the queue",
    }


@app.route("/get/<int:channel>")
def get_x(channel: int):
    return tasks.get(
        channel, {"error": "Channel is not being listened on.", "error_id": 2}
    )


@app.route("/get")
def get():
    return tasks


@app.route("/log/<int:channel>")
def log_x(channel: int):
    if tasks.get(channel) is None:
        return {"error": "Channel is not being listened on.", "error_id": 2}
    logs.append({"channel": channel, "signal": flask.request.args.to_dict()})
    return {
        "signal": flask.request.args,
        "success": "The signal was added to the records",
    }


@app.route("/logs")
def logs_():
    return logs


app.run("0.0.0.0", 8123)
