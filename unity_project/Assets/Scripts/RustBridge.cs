// unity_project/Assets/Scripts/RustBridge.cs

using System;
using System.Runtime.InteropServices;
using UnityEngine;

public class RustBridge : MonoBehaviour
{
    private const string RustLib = "pippop_rust";

    // --- FFI Structs ---
    // These must match the #[repr(C)] structs in ffi.rs exactly.

    [StructLayout(LayoutKind.Sequential)]
    public struct CColor
    {
        public float r, g, b, a;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CPoint
    {
        public double x, y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CGameBubbleData
    {
        public CColor color;
        public uint size;
    }

    [StructLayout(LayoutKind.Explicit)]
    public struct CBubbleKindData
    {
        [FieldOffset(0)]
        public CGameBubbleData game;
    }

    public enum CBubbleKindTag
    {
        Game,
        Player,
        Empty,
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CBubble
    {
        public ulong key_id;
        public CBubbleKindTag kind_tag;
        public CBubbleKindData kind_data;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CRenderableEdge
    {
        public CPoint p0, p1, p2, p3;
        public ulong bubble_a_key_id;
        public ulong bubble_b_key_id;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct CRenderState
    {
        public IntPtr edges;
        public int edge_count;
        private int edges_capacity;

        public IntPtr bubbles;
        public int bubble_count;
        private int bubbles_capacity;
    }

    // --- FFI Function Signatures ---

    [DllImport(RustLib)]
    private static extern IntPtr simulation_new();

    [DllImport(RustLib)]
    private static extern void simulation_free(IntPtr simPtr);

    [DllImport(RustLib)]
    private static extern void simulation_update(IntPtr simPtr, double deltaTime);

    [DllImport(RustLib)]
    private static extern CRenderState simulation_get_render_state(IntPtr simPtr);

    [DllImport(RustLib)]
    private static extern void simulation_free_render_state(CRenderState state);


    // --- Unity MonoBehaviour ---

    private IntPtr simPtr;

    void Start()
    {
        Debug.Log("Initializing Rust simulation...");
        simPtr = simulation_new();
        Debug.Log($"Simulation pointer: {simPtr}");
    }

    void Update()
    {
        if (simPtr == IntPtr.Zero) return;

        simulation_update(simPtr, Time.deltaTime);
        
        CRenderState state = simulation_get_render_state(simPtr);
        
        Debug.Log($"Frame Data: {state.bubble_count} bubbles, {state.edge_count} edges.");

        // In a real game, we would now process the arrays at state.bubbles and state.edges
        // to draw the scene. For now, we just log the counts.

        simulation_free_render_state(state);
    }

    void OnDestroy()
    {
        if (simPtr != IntPtr.Zero)
        {
            Debug.Log("Freeing Rust simulation memory...");
            simulation_free(simPtr);
            simPtr = IntPtr.Zero;
        }
    }
}
