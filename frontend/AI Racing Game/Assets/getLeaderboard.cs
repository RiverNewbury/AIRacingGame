using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.Networking;
using System;

[Serializable]
struct LeaderboardEntry {
	string username;
	Score score;
}

public class getLeaderboard : MonoBehaviour
{
	public int nLeaderboardEntries = 10;
	InfoObject infoObject;

	// Start is called before the first frame update
	void Start()
	{
		infoObject = (InfoObject)UnityEngine.Object.FindObjectOfType(typeof(InfoObject));
	}

	// Update is called once per frame
	void Update()
	{

	}

	void GetLeaderboard() 
	{
		UnityWebRequest getRequest = UnityWebRequest.Get(infoObject.serverAddress + ":8000/leaderboard/" + nLeaderboardEntries);
		getRequest.SendWebRequest();
		if (getRequest.result != UnityWebRequest.Result.Success) {
			Debug.Log(getRequest.error);
		} else {
			Debug.Log("Get request succesful");

			ParseLeaderboard(getRequest.downloadHandler.text);
		}
	}

	void ParseLeaderboard(string leaderboardJson)
	{
		//TODO
		Debug.Log("Unfinished code run in getLeaderboard.cs!!!");
	}
}
