using System;
using System.Runtime.InteropServices;

public static class ffi
{
	#if UNITY_EDITOR

	const string dllName = @"libunity_ffi.dylib";

	[DllImport(dllName)]
	public static extern int print_and_change_value_mac_os (out int x);
	#else

	[DllImport("__Internal")]
	public static extern void print_and_change_value_ios (ref long x); // works with ref but not with out
	#endif
}