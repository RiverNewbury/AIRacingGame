using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class ClickStart : MonoBehaviour
{
    // Start is called before the first frame update
    void NextScene()
    {
        SceneManager.LoadScene(sceneName:"menu");
    }
}
