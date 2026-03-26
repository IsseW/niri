float corner_rounding(vec2 coords, vec2 center, float radius, float scale) {
    float dist = distance(coords, center);
    float half_px = 0.5 / scale;
    return 1.0 - smoothstep(radius - half_px, radius + half_px, dist);
}
