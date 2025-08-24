using System;
using System.Runtime.InteropServices;
using UnityEngine;

// A class to manage the lifecycle of the Rust library and data
public class RustBridge : MonoBehaviour
{
    // --- FFI Definitions ---

    private const string RustLib = "pippop_rust";

    // Structs must match Rust's #[repr(C)] structs

    [StructLayout(LayoutKind.Sequential)]
    public struct Vec2
    {
        public float x;
        public float y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CubicBezier
    {
        public Vec2 p0;
        public Vec2 p1;
        public Vec2 p2;
        public Vec2 p3;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct RenderableBubble
    {
        public ulong bubble_key;
        public bool is_open_air;
        public IntPtr curves; // *mut CubicBezier
        public int curves_count;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct RenderableBubbleCollection
    {
        public IntPtr bubbles; // *mut RenderableBubble
        public int bubbles_count;
    }

    // --- FFI Function Imports ---

    [DllImport(RustLib)]
    private static extern IntPtr create_graph();

    [DllImport(RustLib)]
    private static extern void destroy_graph(IntPtr graphPtr);

    [DllImport(RustLib)]
    private static extern void advance_frame_ffi(IntPtr graphPtr);

    [DllImport(RustLib)]
    private static extern IntPtr get_renderable_bubbles(IntPtr graphPtr); // Returns a pointer to RenderableBubbleCollection

    [DllImport(RustLib)]
    private static extern void free_renderable_bubbles(IntPtr collectionPtr);


    // --- Unity MonoBehaviour ---

    private IntPtr graphPtr;

    void Awake()
    {
        Debug.Log("Initializing Rust graph...");
        graphPtr = create_graph();
        Debug.Log($"Graph pointer: {graphPtr}");
    }

    void Update()
    {
        if (graphPtr == IntPtr.Zero) return;

        // 1. Advance the physics simulation
        advance_frame_ffi(graphPtr);
        
        // 2. Get the renderable data
        IntPtr collectionPtr = get_renderable_bubbles(graphPtr);
        
        if (collectionPtr != IntPtr.Zero)
        {
            // 3. Marshal the data from a pointer to a C# struct
            RenderableBubbleCollection collection = Marshal.PtrToStructure<RenderableBubbleCollection>(collectionPtr);
            
            Debug.Log($"Frame Data: Received {collection.bubbles_count} bubbles from Rust.");

            // In a real game, you would now loop through the bubbles and their curves to draw them.
            // I've added comments to the README with an example of how to do this.

            // 4. Free the memory allocated by Rust for this frame's data
            free_renderable_bubbles(collectionPtr);
        }
    }

    void OnDestroy()
    {
        if (graphPtr != IntPtr.Zero)
        {
            Debug.Log("Freeing Rust graph memory...");
            destroy_graph(graphPtr);
            graphPtr = IntPtr.Zero;
        }
    }
}