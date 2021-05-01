using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.Networking;
using System;

public class getLeaderboard : MonoBehaviour
{
	public int nLeaderboardEntries = 10;
	InfoObject infoObject;

	// Start is called before the first frame update
	void Start()
	{
		infoObject = (InfoObject)UnityEngine.Object.FindObjectOfType(typeof(InfoObject));
		GetLeaderboard();
	}

	// Update is called once per frame
	void Update()
	{

	}

	void GetLeaderboard() 
	{
		UnityWebRequest getRequest = UnityWebRequest.Get(infoObject.serverAddress + ":8000/leaderboard/" + nLeaderboardEntries);
		getRequest.SendWebRequest();

		// wait for response
		WaitForSeconds wait;
		while (!getRequest.isDone) { 
			wait = new WaitForSeconds(0.1f);
		}


		if (getRequest.result != UnityWebRequest.Result.Success) {
			Debug.Log(getRequest.error);
		} else {
			Debug.Log("Get request succesful");
			Debug.Log(getRequest.downloadHandler.text);

			infoObject.ParseLeaderboard(getRequest.downloadHandler.text, nLeaderboardEntries);
		}
	}
}
