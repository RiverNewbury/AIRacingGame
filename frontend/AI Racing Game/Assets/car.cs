using System.Collections;
using System.Collections.Generic;
using System;
using UnityEngine;

// For multithreading
using System.Threading;

// Sockets stuff
using System.Net.Sockets;
using System.Net;

public class car : MonoBehaviour
{
	Thread receiverThread;
	UdpClient client;

	public int port = 59827; // picked randomly by Luca from a port range outside of those that can be registered with IANA

	// Start is called before the first frame update
	void Start()
	{
		receiverThread = new Thread(new ThreadStart(ReceiveData));
		receiverThread.IsBackground = true;
		receiverThread.Start();

		Debug.Log("starting receiverThread");
	}

	// Update is called once per frame
	void Update()
	{
		
	}

	// Function for the thread, listens for packets from the server
	private void ReceiveData()
	{
		try {
			client = new UdpClient(port);
			while (Thread.CurrentThread.IsAlive) {
				var endPoint = new IPEndPoint(IPAddress.Any, 0);//receive from any IP address
				var data = client.Receive(ref endPoint);
				Debug.Log(data);
			}
		} catch (Exception e) { }
	}

	public void OnApplicationQuit()
	{
		if (receiverThread != null) {
			receiverThread.Abort();
		}

		Debug.Log("aborting receiverThread");
	}
}
