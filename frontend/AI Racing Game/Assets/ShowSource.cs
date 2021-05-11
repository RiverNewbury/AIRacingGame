using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.UI;

public class ShowSource : MonoBehaviour
{
	public Text sourceText;
	public GameObject scrollView;
	InfoObject infoObject;

	// Start is called before the first frame update
	void Start()
	{
		sourceText = GameObject.Find("Source text").GetComponent<Text>();
		scrollView = GameObject.Find("Scroll View");
		infoObject = (InfoObject)UnityEngine.Object.FindObjectOfType(typeof(InfoObject));
	}

	// Update is called once per frame
	void Update()
	{

	}

	public void ShowSourceOnClick()
	{
		// show text
		scrollView.SetActive(true);
		//sourceText.gameObject.SetActive(true);

		// process pos
		string rawText = this.GetComponent<Text>().text;
		string posText = rawText.Substring(0, rawText.Length-2);
		int pos = int.Parse(posText);

		// set the text to the source code
		string source = infoObject.leaderboardData.entries[pos-1].source;
		sourceText.text = source;

		// set height and pos of source text
		const int lineHeight = 15;
		int numLines = source.Split('\n').Length;//count lines
		sourceText.gameObject.GetComponent<RectTransform>().sizeDelta = new Vector2(480, lineHeight * numLines);
		sourceText.gameObject.GetComponent<RectTransform>().localPosition = new Vector3(0, (-lineHeight * numLines)/2,0);
	}
}
