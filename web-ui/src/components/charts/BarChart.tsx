import { Bar, BarChart, CartesianGrid } from "recharts"

import { type ChartConfig, ChartContainer } from "@/components/ui/chart"
import React, { useImperativeHandle, useMemo } from "react"
import { cn } from "@/lib/utils"

// example of config 
const _chartConfig = {
  magn: {
    label: "Magnitude",
    color: "green",
  }
} satisfies ChartConfig

export type FftDataType = {
  freq: number; // Frequency band index
  magn: number; // Magnitude of the frequency band
}

export interface BarChartProps extends React.ComponentProps<'div'> {
  chartConfig: ChartConfig;
  data: FftDataType[];
}

export interface BarChartRef {
  setData: (_: FftDataType[]) => void;
}


export const BarChartComponent = React.forwardRef<BarChartRef, BarChartProps>(
  ({ className, chartConfig, data }, ref) => {

    const [currentData, setCurrentData] = React.useState<FftDataType[]>(data);

    useImperativeHandle(ref, () => ({
      setData: (newData: FftDataType[]) => {
        setCurrentData(newData);
      }
    }));

    const classname = useMemo(() => cn(
      "min-h-[200px] w-full",
      className
    ), [className]);

    return (
      <ChartContainer config={chartConfig} className={classname}>
        <BarChart accessibilityLayer data={currentData}>
          <CartesianGrid vertical={true} />
          <Bar dataKey="magn" fill="var(--color-magn)" radius={4} />
        </BarChart>
      </ChartContainer>
    )
  }
);

export default BarChartComponent;
