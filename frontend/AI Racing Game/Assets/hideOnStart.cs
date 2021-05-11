using System.Collections;
using System.Collections.Generic;
using UnityEngine;

using UnityEngine.UI;

public class hideOnStart : MonoBehaviour
{
    void Start()
    {
        this.gameObject.SetActive(false);
    }
}
