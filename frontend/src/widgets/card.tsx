// src/components/MetricCard.tsx
import React from "react";

interface CardProps {
  title?: string;
  value?: number | string;
  className?: string;
  children?: React.ReactNode;
}

const Card: React.FC<CardProps> = ({ title, value, className = "", children }) => {
  // If title and value are provided, use the metric card layout
  if (title && value !== undefined) {
    return (
      <div className={`border p-4 rounded-lg ${className}`}>
        <h2 className="text-xl font-bold mb-2">{title}</h2>
        <p className="text-3xl font-bold">{value}</p>
      </div>
    );
  }

  // Otherwise, render as a container card with children
  return (
    <div className={`border p-4 rounded-lg ${className}`}>
      {children}
    </div>
  );
};

export default Card;
