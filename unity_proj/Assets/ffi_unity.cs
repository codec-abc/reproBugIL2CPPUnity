using System.Collections;
using System.Collections.Generic;
using UnityEngine;

public class ffi_unity : MonoBehaviour 
{

	#if UNITY_EDITOR
	void OnGUI () 
	{
		int x = 11;

		GUI.Label ( new Rect(100, 100, 500, 80), "x was " + x);

		var result = ffi.print_and_change_value_mac_os (out x);

		GUI.Label ( new Rect(100, 200, 500, 80), "result for writting to file is " + result);

		GUI.Label ( new Rect(100, 300, 500, 80), "x is now " + x);
	}
	#else
	void OnGUI () 
	{
		long x = 11;
		GUI.Label ( new Rect(100, 100, 500, 80), "x was " + x);

		ffi.print_and_change_value_ios (ref x);
		GUI.Label ( new Rect(100, 200, 500, 80), "x is now " + x);
	}
	#endif
}
