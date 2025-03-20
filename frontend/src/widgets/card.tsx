// src/components/MetricCard.tsx
import React from "react";

interface CardProps {
  title: string;
  value: number | string;
}

const Card: React.FC<CardProps> = ({ title, value }) => {
  return (
    <div className="border p-4 rounded-lg">
      <h2 className="text-xl font-bold mb-2">{title}</h2>
      <p className="text-3xl font-bold">{value}</p>
    </div>
  );
};

export default Card;
