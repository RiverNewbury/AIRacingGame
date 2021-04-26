using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using AIRacing.Utils;

public class MovingWithWaypoints : MonoBehaviour
{
    public Transform MWW;

    //private GameObject waypoint;

    //public List<GameObject> waypoints = new List<GameObject>();
    //public List<Float> sentwps = ;

    //need to convert sentwps from sent json file
    public class Score {
        public bool success;
        public int time;
    }

    //public static history = InfoObject.history

    public static float[][] sentwps = 
    {
        new[] {0.0f,0.0f},
        new[] {0.0f,-1.0f},
        new[] {0.0f,-2.0f},
        new[] {0.0f,-3.0f},
        new[] {0.0f,-4.0f},
        new[] {0.0f,-5.0f},
        new[] {0.0f,-4.0f},
        new[] {0.0f,-3.0f},
        new[] {0.0f,-2.0f},
        new[] {0.0f,-1.0f},
        new[] {0.0f,0.0f},
        new[] {0.0f,-1.0f},
        new[] {0.0f,-2.0f},
        new[] {0.0f,-3.0f},
        new[] {0.0f,-4.0f},
        new[] {0.0f,-5.0f},
        new[] {0.0f,-4.0f},
        new[] {0.0f,-3.0f},
        new[] {0.0f,-2.0f},
        new[] {0.0f,-1.0f},
        new[] {0.0f,0.0f},
        new[] {0.0f,-1.0f},
        new[] {0.0f,-2.0f},
        new[] {0.0f,-3.0f},
        new[] {0.0f,-4.0f},
        new[] {0.0f,-5.0f},
        new[] {0.0f,-4.0f},
        new[] {0.0f,-3.0f},
        new[] {0.0f,-2.0f},
        new[] {0.0f,-1.0f},
        new[] {0.0f,0.0f},
        new[] {1.0f,0.0f},
        new[] {2.0f,0.0f},
        new[] {3.0f,0.0f},
        new[] {4.0f,0.0f},
        new[] {5.0f,0.0f},
        new[] {6.0f,0.0f},
        new[] {7.0f,0.0f},
        new[] {8.0f,0.0f},
        new[] {9.0f,0.0f},
        new[] {10.0f,0.0f},
        new[] {9.0f,0.0f},
        new[] {8.0f,0.0f},
        new[] {7.0f,0.0f},
        new[] {6.0f,0.0f},
        new[] {5.0f,0.0f},
        new[] {4.0f,0.0f},
        new[] {3.0f,0.0f},
        new[] {2.0f,0.0f},
        new[] {1.0f,0.0f},
        new[] {0.0f,0.0f},
    };
    //public static float[] speed = {1.0f, 5.0f, 12.0f};
    //might need to alter speed
    public float tps = 1;
    public float spt = 1f;
    public Score score = new Score {
        success = true,
        time = 120};
    public static int swpl = sentwps.Length;
    //public Vector2[] waypoints = new Vector2[swpl];
    public float WPRadius = 0.01f;
    public int current = 0;
    public bool crashed = true;

    private static float[] ConvertPos(float[] pos, Vector2 dif)
        {
        float xdif = dif.x;
        float ydif = dif.y;
        float x = pos[0];
        float y = pos[1];
        float SW = 2; //guess for now
        float CW = 0.125f; //ditto
        float SH = 10; //guess for now
        float CH = 4; //ditto
        
        float wx = (CW * x)/SW + xdif;
        float wy = (CH* y)/SH + ydif;
        float[] newpos = new float[] {wx,wy}; 
        return newpos;
    }
    
    // Start is called before the first frame update
    void Start()
    {
       //run at the beginning
        //calculate waypoints from history, so set start to current pos etc.
        Vector2 currentPosition = this.transform.position;
        var i = 0;
        Debug.Log(swpl);
        //Vector2[] _Waypoints = new Vector2[history.Length];
        while (i<swpl){
            sentwps[i] = ConvertPos(sentwps[i], currentPosition);
            //waypoints[i] = new Vector2(sentwps[i][0], sentwps[i][1]);
            i++;
        }
        //Debug.Log(waypoints);
        //i = 0
        //position = waypoints[current].transform.position; 
        //InvokeRepeat("ChangePosition",0,spt)
    }

    //void ChangePosition () {
    //    transform.position = waypoints[i];
    //    i+=1;
    //}

    // Update is called once per frame
    void Update()
    {
        current++;
        if (current >= swpl)
        {
                if (crashed){
                    entryTransform.Find("circle").GetComponent<Image>().color = UtilsClass.GetColorFromString("B76F56");
                }
                else{
                    //show time
                    Debug.Log("done");
                    Debug.Log(current);
                }
                this.enabled = false;//needs to stop here, either say crashed or finished with time
        }
        else {
            //if(Vector2.Distance(waypoints[current].transform.position, transform.position) < WPRadius)
            //{
                Debug.Log(current);
                var newP = new Vector2(sentwps[current][0],sentwps[current][1]);
                transform.position = Vector2.MoveTowards(transform.position, newP, Time.deltaTime * tps);
                //current+=1;
            //}
            if ((current % tps) == 0)
            {
                //update time shown here
            }
        }
    }
}
