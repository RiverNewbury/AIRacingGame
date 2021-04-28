using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using AIRacing.Utils;
using UnityEngine.SceneManagement;

public class MovingWithWaypoints : MonoBehaviour
{
    public Transform MWW;
    //public GameObject EndingText;
    //public Transform EndingText;
    public InfoObject infoObject;
    public SimulationData simData;
    public Score score;
    public History History;
    public Car[] history;
    public int tps;
    public bool crashed;
    public int time;
    public float secTime;
    public static float[,] sentwps;

    public int swpl;
    public float WPRadius = 0.01f;
    public int current = 0;
    
    private static float[] ConvertPos(Point pos, Vector2 dif)
        {
        float xdif = dif.x;
        float ydif = dif.y;
        float x = pos.x;
        float y = pos.y;
        float SW = 2;
        float CW = 0.125f;
        float SH = 10;
        float CH = 4; //these might need to change, estimations
        
        float wx = (CW * x)/SW + xdif;
        float wy = (CH* y)/SH + ydif;
        float[] newpos = new float[] {wx,wy}; 
        return newpos;
    }
    
    // Start is called before the first frame update
    void Start()
    {
        transform.position = new Vector3(-3.53f,0.05f,-2);
        //EndingText = GameObject.Find("TimeOrCrashed");
        infoObject = (InfoObject)UnityEngine.Object.FindObjectOfType(typeof(InfoObject));
        SimulationData simData = infoObject.simulationData;
        History = simData.history;
        score = simData.score;
        tps = History.tps;
        history = History.history;
        //crashed = (score.successful==false);
        crashed = true;
        swpl = history.Length;
        time = score.time;
        secTime = time/tps;
        sentwps = new float[swpl,2];
        
        
        
        Vector2 currentPosition = this.transform.position;
        var i = 0;
        while (i<swpl){
            var temp = ConvertPos(history[i].pos, currentPosition);
            sentwps[i,0]= temp[0];
            sentwps[i,1]= temp[1];
            i++;
        }
    }

    void Update()
    {
        current++;
        if (current >= swpl)
        {
                if (crashed){
                    Debug.Log("crashed");
                    MWW.Find("Circle").GetComponent<SpriteRenderer>().color = UtilsClass.GetColorFromString("FF0000");
                }
                else{
                    //EndingText.Find("TimeOrCrashed").GetComponent<Text>().text = secTime.ToString();
                    //show time
                    //Debug.Log("done");
                    //Debug.Log(current);
                }
                //EndingText.Find("TimeOrCrashed").GetComponent<Text>().SetActive(true);
                IEnumerator LoadLevelAfterDelay(){
                yield return new WaitForSeconds(7);
                SceneManager.LoadScene(sceneName:"GameScene_HighscoreTable");
                }
                LoadLevelAfterDelay();
                this.enabled = false;//don't know if this is necessary
        }
        else {
            var newP = new Vector2(sentwps[current,0],sentwps[current,1]);
            transform.position = Vector2.MoveTowards(transform.position, newP, Time.deltaTime * tps);

        }
    }
}
