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

public class InfoObject : MonoBehaviour
{
	//static public History history;
	//static public Score score;
	public SimulationData simulationData;
	public string serverAddress;

	void Awake()
	{
		DontDestroyOnLoad(this);
	}

	// turn JSON string into data stored in this object
	public void ParseHistory(string historyJson)
	{
		simulationData = JsonUtility.FromJson<SimulationData>(historyJson);
	}
}
