using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System;
using System.Numerics;

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
public struct LeaderboardEntry {
	public string username;
	public Score score;
}

[Serializable]
public struct LeaderboardData {
	public LeaderboardEntry[] entries;
}

public class InfoObject : MonoBehaviour
{
	public SimulationData simulationData;
	public LeaderboardData leaderboardData;
	public int n_entries;//this is not necessary the total amount fetched, only the amount requested. "simulationData.entries.Length" will get the actual number for you
	public string serverAddress;

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
		n_entries = n;
	}
}
