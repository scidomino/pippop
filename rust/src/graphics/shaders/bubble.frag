#version 100
precision mediump float;

varying lowp vec4 color;
varying lowp vec2 uv;

void main() {
    // UV is 0.0 to 1.0. Center is 0.5, 0.5.
    vec2 rel = uv - 0.5;
    
    // 1. Specular Highlight (Crescent Shape in Top-Left)
    vec2 arc_center = vec2(0.04, 0.04);
    vec2 rel_arc = rel - arc_center;
    float r = length(rel_arc);
    float arc_r = 0.38;
    float dist = abs(r - arc_r);
    float ring = pow(max(0.0, 1.0 - dist / 0.11), 2.0);
    
    vec2 rel_norm = rel_arc / (r + 0.0001);
    vec2 light_dir = normalize(vec2(-1.0, -1.0));
    float dot_p = dot(rel_norm, light_dir);
    float angle_fade = pow(max(0.0, dot_p), 16.0);
    
    float spec = ring * angle_fade;
    
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
