#version 450

layout(location = 0) out vec4 out_color;

void main() {
    float dist_to_center = distance(gl_FragCoord.xy, vec2(0.5, 0.5));
    float circle_radius = 0.3;
    float circle_thickness = 0.1;

    if(dist_to_center > circle_radius - circle_thickness && dist_to_center < circle_radius + circle_thickness) {
        out_color = vec4(1.0, 0.0, 0.0, 1.0); // Punainen
    } else {
        out_color = vec4(0.0, 0.0, 0.0, 1.0); // Musta
    }
}
