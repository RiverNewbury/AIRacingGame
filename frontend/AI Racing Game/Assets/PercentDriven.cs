using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class PercentDriven : MonoBehaviour
{
        //or whatever the file will be called
        //assuming file name historyFile
        //startPos = [6,5]
        //boolean [12,37] boolTrack = [[False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False], [False, False, False, False, False, False, False, False, False, False, False, False, False, False, True, True, True, True, True, True, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False], [False, False, False, False, False, False, False, False, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, False, False, False, False, False, False, False, False, False], [False, False, False, False, False, True, True, True, True, True, True, True, False, False, False, False, False, False, False, False, False, False, False, True, True, True, True, True, True, True, True, False, False, False, False, False, False], [False, False, False, False, True, True, True, True, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, True, True, True, True, True, False, False, False, False, False], [False, False, False, False, True, True, True, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, True, True, True, True, False, False, False, False], [False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, True, True, True, False, False, False], [False, False, False, False, True, True, True, True, False, False, False, False, False, True, True, True, True, True, True, True, True, True, False, False, False, False, False, False, False, False, False, True, True, True, False, False, False], [False, False, False, False, False, True, True, True, True, True, True, True, True, True, True, True, True, False, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, True, False, False, False], [False, False, False, False, False, False, False, False, False, False, True, True, True, True, False, False, False, False, False, False, False, False, False, False, False, False, True, True, True, True, True, True, True, False, False, False, False], [False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False], [False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False, False]]
        //string json = File.ReadAllText("historyFile.json");
        //var playerLap = JsonConvert.DeserializeObject<List<Player>>(json);
        //public GameObject[] waypoints;
        int current = 0;
        float rotSpeed;
        public float speed;
        float WPradius = 1;
        
        void Start(){
            //run at the beginning
            //calculate waypoints from history, so set start to current pos etc.
        }

        void Update() {
            if(Vector2.Distance(waypoints[current].transform.position, transform.poistion) < WPradius)
            {
                current++;
                if (current >= waypoints.Length)
                {
                    //needs to stop here, either say crashed or finished with time
                }
                transform.position = Vector2.MoveTowards(transform.position, waypoints[current].transform.position, Time.deltaTime*speed);
            }
        }
}
