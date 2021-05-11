using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class closeOnClick : MonoBehaviour
{
	public void close()
	{
		this.gameObject.SetActive(false);
	}
}
