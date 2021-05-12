using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System;
using System.Numerics;
using UnityEngine.SceneManagement;
using UnityEngine.Networking;
using UnityEngine.UI;
using TMPro;


public class sendCode : MonoBehaviour
{
	public TMP_InputField codeField;
	public InputField usernameField;
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

	public void SendCode()
	{
		if (usernameField.text == "") {
			Debug.Log("No username");
		} else {
			UnityWebRequest postRequest = UnityWebRequest.Post(infoObject.serverAddress + ":8000/run/" + usernameField.text, codeField.text.Substring(16));
			Debug.Log(codeField.text.Substring(16));
			postRequest.timeout = 20;
			postRequest.SendWebRequest();

			// wait for response
			WaitForSeconds wait;
			while (!postRequest.isDone) { 
				wait = new WaitForSeconds(0.1f);
			}

			if (postRequest.result != UnityWebRequest.Result.Success) {
				Debug.Log(postRequest.error);
			} else {
				Debug.Log("Post request successful");
				Debug.Log(postRequest.downloadHandler.text);

				infoObject.ParseHistory(postRequest.downloadHandler.text);
				SceneManager.LoadScene(sceneName:"CarSimulation");
			}
		}
	}
}
