using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.EventSystems;
using UnityEngine.Events;

public class startButton : MonoBehaviour, IPointerClickHandler
{
    public Transform text;
    public float timer;
    public GameObject Tap;
    public static bool activated = false;
    int UILayer;
    public UnityEvent onClick;

    public bool IsPointerOverUIElement()
    {
        return IsPointerOverUIElement(GetEventSystemRaycastResults());
    }
 
    public void OnPointerClick(PointerEventData pointerEventData)
    {
        //Output to console the clicked GameObject's name and the following message. You can replace this with your own actions for when clicking the GameObject.
        Debug.Log(name + " Game Object Clicked!", this);

        // invoke your event
        onClick.Invoke();
    }
 
    //Returns 'true' if we touched or hovering on Unity UI element.
    private bool IsPointerOverUIElement(List<RaycastResult> eventSystemRaysastResults)
    {
        for (int index = 0; index < eventSystemRaysastResults.Count; index++)
        {
            RaycastResult curRaysastResult = eventSystemRaysastResults[index];
            if (curRaysastResult.gameObject.layer == UILayer)
                return true;
        }
        return false;
    }
 
 
    //Gets all event system raycast results of current mouse or touch position.
    static List<RaycastResult> GetEventSystemRaycastResults()
    {
        PointerEventData eventData = new PointerEventData(EventSystem.current);
        eventData.position = Input.mousePosition;
        List<RaycastResult> raysastResults = new List<RaycastResult>();
        EventSystem.current.RaycastAll(eventData, raysastResults);
        return raysastResults;
    }

    void exit()
    {
       if (Input.touchCount > 0 && activated == false)
        {
            Tap.SetActive(false);
            activated = true;
        }
    }
    void Start()
    {
        UILayer = LayerMask.NameToLayer("UI");
    }

    // Update is called once per frame
    void Update()
    {
        exit();
           timer = timer + Time.deltaTime;
           if(IsPointerOverUIElement()){
               GetComponent<Text>().enabled = true;
           }
           else{
           if(timer >= 0.5)
           {
                   GetComponent<Text>().enabled = true;
           }
           if(timer >= 1)
           {
                   GetComponent<Text>().enabled = false;
                   timer = 0;
           }
           }
    }
}
