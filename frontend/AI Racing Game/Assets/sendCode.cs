using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using System;
using System.Numerics;
using UnityEngine.SceneManagement;
using UnityEngine.Networking;
using UnityEngine.UI;

public class sendCode : MonoBehaviour
{
	public InputField codeField;
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
		WWWForm form = new WWWForm();
		form.AddField("source_code", codeField.text);

		UnityWebRequest postRequest = UnityWebRequest.Post(infoObject.serverAddress + ":8000/run/" + usernameField.text, form);
		postRequest.SendWebRequest();

		if (postRequest.result != UnityWebRequest.Result.Success) {
			Debug.Log(postRequest.error);
		} else {
			Debug.Log("Post request successful");

			infoObject.ParseHistory(postRequest.downloadHandler.text);
			SceneManager.LoadScene(sceneName:"CarSimulation");
		}
	}
}
