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
	public string serverAddress;


	// Start is called before the first frame update
	void Start()
	{

	}

	// Update is called once per frame
	void Update()
	{

	}

	public void SendCode()
	{
		WWWForm form = new WWWForm();
		form.AddField("source_code", codeField.text);

		UnityWebRequest postRequest = UnityWebRequest.Post(serverAddress + ":8000/run/" + usernameField.text, form);
		postRequest.SendWebRequest();

		//if (postRequest.result != UnityWebRequest.Result.Success) {
			Debug.Log(postRequest.error);
		//} else {
			Debug.Log("Form upload complete!");

			Debug.Log(postRequest.downloadHandler.text);
			InfoObject.ParseHistory(postRequest.downloadHandler.text);
			SceneManager.LoadScene(sceneName:"simulation");
		//}
	}
}
