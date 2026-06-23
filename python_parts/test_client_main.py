import requests
from urllib.parse import urljoin

TEST_SERVER = "http://localhost:5000"

# pytest --capture=no # to see output


def test_minimal():
    session = requests.Session()
    response = session.get(urljoin(TEST_SERVER, '/'))
    pretty_response(response)
    assert response.cookies.get_dict() == {}

    response = session.post(urljoin(TEST_SERVER, "set_cookie"))
    pretty_response(response)
    response.cookies.get_dict() == {'almost_session', 'very_complicated_random_string'}

    response = session.get(urljoin(TEST_SERVER, '.'))
    pretty_response(response)
    assert response.request.headers.get('Cookie') == "almost_session=very_complicated_random_string"



# TODO: this is not pretty at all
def pretty_response(response):
    print(f"""
METHOD: {response.request.method}
URL: {response.url}

STATUS: {response.status_code}

REQUEST COOKIES:
{response.request.headers.get('Cookie')}

RESPONSE COOKIES:
{response.cookies.get_dict()}

REQUEST HEADERS:
{dict(response.request.headers)}

RESPONSE HEADERS:
{dict(response.headers)}

BODY:
{response.text[:5000]}
""")
