using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.Networking;
using UnityEngine.UI;


public class sendCode : MonoBehaviour
{
	public InputField codeField;
	public InputField usernameField;
	public String serverAddress;


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
		// Modified from example code at docs.unity3d.com/Manual/UnityWebRequest-SendingForm.html
		WWWForm form = new WWWForm();
		form.AddField("username", usernameField.text);
		form.AddField("code", codeField.text);

		UnityWebRequest www = UnityWebRequest.Post(serverAddress, form);
		yield return www.SendWebRequest();

		if (www.result != UnityWebRequest.Result.Success) {
			Debug.Log(www.error);
		}
		else {
			Debug.Log("Form upload complete!");
		}
	}
}
