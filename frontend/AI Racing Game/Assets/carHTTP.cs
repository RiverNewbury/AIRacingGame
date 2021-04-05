using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.Networking;

public class carHTTP : MonoBehaviour
{
	public string serverAddress;

	// Start is called before the first frame update
	void Start()
	{
		// Get request
		UnityWebRequest webRequest = UnityWebRequest.Get(serverAddress);

		if (webRequest.result != UnityWebRequest.Result.Success) {
			Debug.Log(webRequest.error);
		} else {
			// Show results as text
			Debug.Log(webRequest.downloadHandler.text);

			// Or retrieve results as binary data
			byte[] results = webRequest.downloadHandler.data;
		}
	}

	// Update is called once per frame
	void Update()
	{

	}
}
