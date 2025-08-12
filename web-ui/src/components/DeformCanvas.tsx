import { Canvas, extend, useFrame } from '@react-three/fiber';
import { shaderMaterial } from '@react-three/drei';
import React, { useRef, type JSX } from 'react';
import * as THREE from 'three';

// --- Typage des uniforms ---
type DeformMaterialUniforms = {
    time: { value: number };
    audioLevel: { value: number };
};

// Création du shaderMaterial
const DeformMaterialImpl = shaderMaterial(
    { time: 0, audioLevel: 0 },
  /* glsl */ `
    uniform float time;
    uniform float audioLevel;
    varying vec2 vUv;

    void main() {
      vUv = uv;
      vec3 pos = position;
      pos.z += sin(pos.x * 5.0 + time) * audioLevel * 0.5;
      gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
    }
  `,
  /* glsl */ `
    varying vec2 vUv;
    void main() {
      gl_FragColor = vec4(vUv.x, vUv.y, 0.5, 1.0);
    }
  `
);

extend({ DeformMaterialImpl });

// --- Déclaration JSX ---
declare module '@react-three/fiber' {
    interface ThreeElements {
        deformMaterialImpl: JSX.IntrinsicElements['shaderMaterial'] & {
            ref?: React.Ref<THREE.ShaderMaterial & { uniforms: DeformMaterialUniforms }>;
        };
    }
}

function DeformedPlane({ audioLevel }: { audioLevel: number }) {
    const matRef = useRef<THREE.ShaderMaterial & { uniforms: DeformMaterialUniforms }>(null!);

    useFrame(({ clock }) => {
        if (matRef.current) {
            matRef.current.uniforms.time.value = clock.getElapsedTime();
            matRef.current.uniforms.audioLevel.value = audioLevel;
        }
    });

    return (
        <mesh>
            <sphereGeometry args={[3, 32, 32]} />
            <deformMaterialImpl ref={matRef} />
        </mesh>
    );
}

export function DeformCanvas({ audioLevel }: { audioLevel: number }) {
    return (
        <Canvas camera={{ position: [0, 0, 5] }}>
            <ambientLight />
            <DeformedPlane audioLevel={audioLevel} />
        </Canvas>
    );
}
