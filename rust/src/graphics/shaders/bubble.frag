#version 100
precision mediump float;

varying lowp vec4 color;
varying lowp vec2 uv;

void main() {
    // UV is 0.0 to 1.0. Center is 0.5, 0.5.
    vec2 rel = uv - 0.5;
    
    // 1. Specular Highlight (Top-Left)
    vec2 spec_pos = vec2(0.3, 0.3);
    float spec = pow(max(0.0, 1.0 - length(uv - spec_pos) / 0.15), 32.0);
    
    // 2. Directional Shading (Light from Top-Left, Shadow on Bottom-Right)
    vec2 shadow_offset = vec2(-0.2, -0.2);
    float shadow_d = length(rel - shadow_offset) * 1.5;
    float shadow = pow(clamp(shadow_d, 0.0, 1.0), 2.0);
    
    // Solid Color Logic
    vec3 base_tint = color.rgb;
    
    // Apply Directional Shading
    vec3 final_rgb = mix(base_tint, base_tint * 0.35, shadow);
    
    // Add specular highlight (bright white), but fade it with the bubble's alpha
    final_rgb += vec3(spec);
    
    // Simple, consistent alpha handling
    float alpha = color.a;

    final_rgb = clamp(final_rgb, 0.0, 1.0);
    
    // Apply pre-multiplied alpha for smooth blending
    gl_FragColor = vec4(final_rgb * alpha, alpha);
}
