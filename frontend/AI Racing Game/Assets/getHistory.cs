using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.Networking;

public class getHistory : MonoBehaviour
{
	public string serverAddress;

	// Start is called before the first frame update
	void Start()
	{
		UnityWebRequest getRequest = UnityWebRequest.Get(serverAddress);
		getRequest.SendWebRequest();

		if (getRequest.result != UnityWebRequest.Result.Success) {
			Debug.Log(getRequest.error);
		} else {
			// string containing history and leaderboard in JSON format
			string serverData = getRequest.downloadHandler.text;

			Debug.Log(serverData);
		}
	}

	// Update is called once per frame
	void Update()
	{

	}
}
