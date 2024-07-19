from fhirclient import client
import json

settings = {
    'app_id': 'your_app_id',
    'api_base': 'https://r4.smarthealthit.org',  # SMART Health IT Sandbox
    'redirect_uri': 'http://localhost:5000/fhir_callback'
}

smart = client.FHIRClient(settings=settings)

def fetch_patient():
    if smart.ready:
        patient = smart.server.request_json('Patient/SMART-1288992')
        return json.dumps(patient, indent=4)
    return json.dumps({'error': 'Not authorized or not ready'}, indent=4)

if __name__ == '__main__':
    print(fetch_patient())
