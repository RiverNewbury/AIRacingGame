using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System;
using System.Numerics;
using UnityEngine.Networking;

[Serializable]
public struct Score {
	public bool successful;
	public int time;
}

[Serializable]
public struct Point {
	public float x;
	public float y;
}

[Serializable]
public struct Car {
	public Point pos;
	public float angle;
	public float speed;
	//public float max_turn;
}

[Serializable]
public struct History {
	public Car[] history;
	public int tps;
}

[Serializable]
public struct SimulationData {
	public History history;
	public Score score;
}

[Serializable]
public struct RankedSource {
	public string username;
	public Score score;
	public string source;
}

[Serializable]
public struct LeaderboardData {
	public RankedSource[] entries;
}

public class InfoObject : MonoBehaviour
{
	public SimulationData simulationData;
	public LeaderboardData leaderboardData;
	public int nLeaderboardEntries = 10;//this is not necessary the total amount fetched, only the amount requested. "simulationData.entries.Length" will get the actual number for you
	public string serverAddress;
	public bool leaderboardFetched = false;

	void Awake()
	{
		DontDestroyOnLoad(this);
	}

	// turn JSON string into data stored in this object
	public void ParseHistory(string json)
	{
		simulationData = JsonUtility.FromJson<SimulationData>(json);
	}
	
	// turn JSON string into data stored in this object
	public void ParseLeaderboard(string json, int n)
	{
		leaderboardData = JsonUtility.FromJson<LeaderboardData>("{\"entries\":" + json + "}");
		leaderboardFetched = true;
	}

	public LeaderboardData GetLeaderboard() 
	{
		UnityWebRequest getRequest = UnityWebRequest.Get(serverAddress + ":8000/leaderboard/" + nLeaderboardEntries);
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

			this.ParseLeaderboard(getRequest.downloadHandler.text, nLeaderboardEntries);
		}

		return leaderboardData;
	}
}
