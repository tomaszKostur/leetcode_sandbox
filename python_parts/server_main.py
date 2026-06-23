from flask import Flask, make_response

app = Flask(__name__)

@app.route("/")
def hello():
    return "Hello, World!"

@app.route("/set_cookie", methods=['POST'])
def set_cookie():
    response = make_response("Logged in")
    response.set_cookie(key='almost_session', value='very_complicated_random_string')
    return response

if __name__ == "__main__":
    app.run(debug=True)