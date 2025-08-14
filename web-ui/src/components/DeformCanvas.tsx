import { Canvas, extend, useFrame } from '@react-three/fiber';
import { shaderMaterial } from '@react-three/drei';
import React, { useRef, type JSX } from 'react';
import * as THREE from 'three';

const NUM_BANDS = 128;

const DeformMaterialImpl = shaderMaterial(
    { time: 0, audioLevel: 0, audioBands: new Array(NUM_BANDS).fill(0) },
  /* vertex shader */ `
    uniform float time;
    uniform float audioLevel;
    uniform float audioBands[${NUM_BANDS}];
    varying vec2 vUv;

    void main() {
      vUv = uv;
      vec3 pos = position;

      // Map pos.x from [-1,1] to [0,NUM_BANDS-1]
      float bandIndex = clamp(floor((pos.x + 1.0) * 0.5 * float(${NUM_BANDS})), 0.0, float(${NUM_BANDS}-1));
      float bandValue = audioBands[int(bandIndex)];

      pos.z += cos(pos.x * 5.0 + time) * bandValue;
      gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
    }
  `,
  /* fragment shader */ `
    varying vec2 vUv;
    void main() {
      gl_FragColor = vec4(vUv.x, vUv.y, 0.5, 1.0);
    }
  `
);

extend({ DeformMaterialImpl });

type DeformMaterialType = THREE.ShaderMaterial & {
    time: number;
    audioLevel: number;
    audioBands: number[];
};

// Register the material type so it can be used in JSX
declare module '@react-three/fiber' {
    interface ThreeElements {
        deformMaterialImpl: JSX.IntrinsicElements['shaderMaterial'] & {
            ref?: React.Ref<DeformMaterialType>;
        };
    }
}

function DeformedPlane({ audioLevel, magnitudes }: { audioLevel: number; magnitudes: number[] }) {
    const matRef = useRef<DeformMaterialType>(null);
    const elemRef = useRef<THREE.SphereGeometry>(null);

    useFrame((state, delta) => {
        if (elemRef.current) {
            elemRef.current.rotateY(delta * 0.5);
        }
        if (matRef.current) {
            matRef.current.time = state.clock.getElapsedTime();
            matRef.current.audioLevel = audioLevel;
            matRef.current.audioBands = magnitudes;
        }
    });

    return (
        <mesh>
            <sphereGeometry args={[4, 100, 100]} ref={elemRef} />
            <deformMaterialImpl ref={matRef} />
        </mesh>
    );
}

export function DeformCanvas({ audioLevel, magnitudes }: { audioLevel: number; magnitudes: number[] }) {
    return (
        <Canvas camera={{ position: [0, 0, 10] }} >
            <ambientLight />
            <DeformedPlane audioLevel={audioLevel} magnitudes={magnitudes} />
        </Canvas>
    );
}
