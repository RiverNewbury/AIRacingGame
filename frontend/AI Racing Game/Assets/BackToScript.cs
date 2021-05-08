using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;

public class BackToScript : MonoBehaviour
{
    public void toScript()
    {
        SceneManager.LoadScene(sceneName: "menu");
    }
}
