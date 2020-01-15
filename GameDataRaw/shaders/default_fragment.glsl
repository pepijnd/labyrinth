#version 330 core

in vec2 v_tex_coords;
in vec3 v_normal;
in vec3 v_position;
in vec4 v_shadow_coords;
in mat3 tbn;
in vec4 v_bones;

out vec4 color;

const vec3 lightd = vec3(0.1, -0.1, 1.0);
const vec3 lightc = vec3(1.0, 1.0, 1.0);
const vec3 global = vec3(0.4, 0.4, 0.4);

uniform sampler2DShadow shadow_map;
//uniform sampler2D tex;
//uniform sampler2D normal_map;
uniform vec3 camera_pos;

struct Material {
    vec3 emission;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
    float refraction;
    float alpha;
};

struct Light {
    vec3 position;
    vec3 direction;
    vec3 color;
};

layout(std140) uniform matmap {
    Material material;
};

layout(std140) uniform lightmap {
    Light light;
};

const vec2 poissonDisk[16] = vec2[]( 
vec2( -0.94201624, -0.39906216 ), 
vec2( 0.94558609, -0.76890725 ), 
vec2( -0.094184101, -0.92938870 ), 
vec2( 0.34495938, 0.29387760 ), 
vec2( -0.91588581, 0.45771432 ), 
vec2( -0.81544232, -0.87912464 ), 
vec2( -0.38277543, 0.27676845 ), 
vec2( 0.97484398, 0.75648379 ), 
vec2( 0.44323325, -0.97511554 ), 
vec2( 0.53742981, -0.47373420 ), 
vec2( -0.26496911, -0.41893023 ), 
vec2( 0.79197514, 0.19090188 ), 
vec2( -0.24188840, 0.99706507 ), 
vec2( -0.81409955, 0.91437590 ), 
vec2( 0.19984126, 0.78641367 ), 
vec2( 0.14383161, -0.14100790 ) 
);

float random(vec4 seed4) {
    float dot_product = dot(seed4, vec4(12.9898,78.233,45.164,94.673));
    return fract(sin(dot_product) * 43758.5453);
}

void main() {
    // vec3 normal = texture(normal_map, v_tex_coords).rgb;
    //      normal = normalize(normal * 2.0 - 1.0);
    //      normal = normalize(tbn * normal);
    vec3 normal = normalize(v_normal);

    // vec3 toLight = normalize(light.position - v_position);
    // float thetaLight = dot(toLight, normal);

    vec3 tex_color = vec3(1.0);
    // for (int i=0; i<8; i++) {
    //     int index = int(16.0*random(vec4(v_position.xyy, i))) % 16;
    //     tex_color += texture(tex, v_tex_coords + poissonDisk[index] / 60.0).rgb / 8;
    // }
    //tex_color = texture(tex, v_tex_coords).rgb;
    vec3 diffuse_color = tex_color * material.diffuse;
    vec3 ambient_color = diffuse_color * material.ambient * 0.10;
    vec3 specular_color = material.specular;

    // float slopeScale = clamp(1.0 - thetaLight, 0.0, 1.0);
    // vec4 shadow_coords = v_shadow_coords;// + offset;
    // float bias = tan(acos(dot(normal, normalize(light.position - light.center)))) * 0.0005;
    // bias = clamp(bias, 0.0002, 0.005);
    // float visibility = 0.0;
    // float texelSize = 1.0 / textureSize(shadow_map, 0).x;
    // float div = 61440.0 * texelSize;
    // for (int i=0; i<8; i++) {
    //     int index = int(16.0*random(vec4(v_position.xyy, i))) % 16;
    //     visibility += textureProj(shadow_map, vec4(shadow_coords.xy + poissonDisk[index] / div, shadow_coords.z - bias, shadow_coords.w)) / 8.0;
    // }
    // if (v_shadow_coords.w < 0.0) {
    //     visibility = 0.0;
    // }
    // visibility = clamp(visibility, 0.5, 1.0);
    float dist = distance(light.position, v_position);
    float att = 1.0 / (1.0 + 0.1*dist + 0.01*dist*dist);

    vec3 light_dir = normalize(light.position - v_position);

    float diffuse = clamp(dot(normal, light.direction), 0.0, 1.0);

    vec3 camera_dir = normalize(v_position - camera_pos);
    vec3 half_direction = normalize(light.direction + camera_dir);
    //float specular = pow(clamp(dot(half_direction, normal), 0.0, 1.0), material.shininess);

    vec3 out_color = ambient_color;

    if (att * max(0.0, dot(normal, light_dir)) > 0.1) {
        out_color = diffuse_color * light.color;
    }

    if (dot(normal, light_dir) > 0.0
        && att * pow(max(0.0, dot(
            reflect(-light_dir, normal),
            camera_dir)), 1) > 0.5 ) 
    {
        out_color = specular_color * light.color;
    } 
    
    color = vec4(
        out_color,
        1.0
    );

    color.a *= material.alpha;
    color = normalize(v_bones);
    color.a = 1;
}