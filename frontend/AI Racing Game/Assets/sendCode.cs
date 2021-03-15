using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.UI;


public class sendCode : MonoBehaviour
{
	public InputField codeField;
	public InputField usernameField;


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
		// Dummy code that just tests it
		Debug.Log(codeField.text);
		Debug.Log(usernameField.text);

		// Need network code to send it off to the server here
	}
}
