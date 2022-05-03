from configparser import ConfigParser
from google.cloud import secretmanager

def local() -> ConfigParser:
    config = ConfigParser()
    config.read('config.ini')
    return config

def secret(secret_id):
    gcloud_config = local()['gcloud']
    project_id = gcloud_config['project_id']
    version_id = gcloud_config['secret_version']

    client = secretmanager.SecretManagerServiceClient()
    name = f"projects/{project_id}/secrets/{secret_id}/versions/{version_id}"
    response = client.access_secret_version(request={"name": name})

    payload = response.payload.data.decode("UTF-8")
    return payload
