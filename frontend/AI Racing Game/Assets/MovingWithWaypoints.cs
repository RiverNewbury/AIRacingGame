using System.Collections;
using System.Collections.Generic;
using UnityEngine;

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

    public static float[][] sentwps = 
    {
        new[] {0.0f,0.0f},
        new[] {0.5f,0.5f},
        new[] {1.0f,1.0f}
    };
    public static float[] speed = {1.0f, 5.0f, 12.0f};
    //might need to alter speed
    public int tps = 100;
    public Score score = new Score {
        success = true,
        time = 120}
    ;
    public static int swpl = sentwps.Length;
    public Vector2[] waypoints = new Vector2[swpl];
    public float WPRadius = 0.01f;
    public int current = 0;
    public bool crashed = false;

    private static float[] ConvertPos(float[] pos, Vector2 dif)
        {
        float xdif = dif.x;
        float ydif = dif.y;
        float x = pos[0];
        float y = pos[1];
        float SW = 10; //guess for now
        float CW = 10; //ditto
        float SH = 10; //guess for now
        float CH = 10; //ditto
        float wx = (CW * x)/SW + xdif;
        float wy = (CH* x)/SH + ydif;
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
        //Vector2[] _Waypoints = new Vector2[history.Length];
        while (i<swpl){
            sentwps[i] = ConvertPos(sentwps[i], currentPosition);
            waypoints[i] = new Vector2(waypoints[i][0], waypoints[i][1]);
            i++;
        }
        //position = waypoints[current].transform.position; 
    }

    // Update is called once per frame
    void Update()
    {
        current++;
        if (current >= waypoints.Length)
        {
                if (crashed){
                    //animation for crash
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
                transform.position = Vector2.MoveTowards(transform.position, waypoints[current], Time.deltaTime*speed[current]);
            //}
            if ((current % tps) == 0)
            {
                //update time shown here
            }
        }
    }
}
