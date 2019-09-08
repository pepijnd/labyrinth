#version 330 core

in vec2 v_tex_coords;
in vec3 v_normal;
in vec3 v_position;
in vec4 v_shadow_coords;

out vec4 color;

const vec3 lightd = vec3(0.1, -0.1, 1.0);
const vec3 lightc = vec3(1.0, 1.0, 1.0);
const vec3 global = vec3(0.4, 0.4, 0.4);

uniform sampler2DShadow shadow_map;
uniform sampler2D tex;
uniform sampler2D normal;
uniform vec3 camera_pos;

struct Material {
    float specular_coefficient;
    vec3 color_ambient;
    vec3 color_diffuse;
    vec3 color_specular;
    vec3 color_emissive;
    float optical_density;
    float alpha;
};

struct Light {
    vec3 position;
    vec3 center;
    vec3 color;
};

layout(std140) uniform matmap {
    Material material;
};

layout(std140) uniform lightmap {
    Light light;
};

mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
    vec3 dp1 = dFdx(pos);
    vec3 dp2 = dFdy(pos);
    vec2 duv1 = dFdx(uv);
    vec2 duv2 = dFdy(uv);

    vec3 dp2perp = cross(dp2, normal);
    vec3 dp1perp = cross(normal, dp1);
    vec3 T = dp2perp * duv1.x + dp1perp * duv2.x;
    vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
    return mat3(T * invmax, B * invmax, normal);
}

const vec2 poissonDisk[4] = vec2[](
  vec2( -0.94201624, -0.39906216 ),
  vec2( 0.94558609, -0.76890725 ),
  vec2( -0.094184101, -0.92938870 ),
  vec2( 0.34495938, 0.29387760 )
);

float random(vec4 seed4) {
    float dot_product = dot(seed4, vec4(12.9898,78.233,45.164,94.673));
    return fract(sin(dot_product) * 43758.5453);
}

void main() {
    vec3 n_normal = v_normal;
    if (gl_FrontFacing) {
        n_normal *= -1;  
    };
    vec3 normal_map = texture(normal, v_tex_coords).rgb;
    mat3 tbn = cotangent_frame(n_normal, v_position, v_tex_coords);
    vec3 real_normal = normalize(tbn * -(normal_map * 2.0 - 1.0));

    vec3 tex_color = texture(tex, v_tex_coords).rgb;
    vec3 diffuse_color = tex_color * material.color_diffuse;
    vec3 ambient_color = diffuse_color * material.color_ambient;
    vec3 specular_color = material.color_specular;

    float lum = max(dot(normalize(n_normal), normalize(light.position)), 0.0);
    float bias = clamp(0.005*tan(acos(lum)), 0.0, 0.01);
    float visibility = 1.0;
    for (int i=0;i<4;i++){
        int index = int(16.0*random(vec4(gl_FragCoord.xyy, i)))%16;
        visibility *= texture(shadow_map, vec3(v_shadow_coords.xy + poissonDisk[index]/40000.0, v_shadow_coords.z-bias), 0.1);
    }
    visibility = max(lum * visibility, 0.05);

    float diffuse = min(max(dot(normalize(real_normal), normalize(light.position-light.center)), 0.0), 0.9);

    vec3 camera_dir = normalize(v_position - camera_pos);
    vec3 half_direction = normalize(normalize(light.position-light.center) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(real_normal)), 0.0), material.specular_coefficient);

    color = vec4(
        ambient_color +
        visibility * diffuse_color * light.color * lum +
        visibility * specular * specular_color * light.color
        , 1.0);
    color.a *= material.alpha;
}