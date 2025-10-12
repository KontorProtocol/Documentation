import React from 'react';
import { LineChart, Line, Area, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';

const KORSupplyGrowth = () => {
  // Starting supply (in millions)
  const initialSupply = 1000;
  
  // Calculate supply over 10 years at 2% and 5% growth rates
  const data = Array.from({ length: 11 }, (_, year) => {
    const growth2 = initialSupply * Math.pow(1.02, year);
    const growth5 = initialSupply * Math.pow(1.05, year);
    
    return {
      year,
      '2% Growth': Math.round(growth2 * 100) / 100,
      '5% Growth': Math.round(growth5 * 100) / 100,
    };
  });

  return (
    <div className="w-full h-screen flex flex-col items-center justify-center p-8" style={{ fontFamily: 'Inter, sans-serif' }}>
      <div className="rounded-lg p-8 w-full max-w-6xl">
        <h1 className="text-3xl font-bold text-slate-800 mb-2 text-center">
          KOR Cryptocurrency Supply Growth
        </h1>
        <p className="text-slate-600 mb-6 text-center">
          10-Year Projection at 2-5% Annual Growth Rates
        </p>
        
        <ResponsiveContainer width="100%" height={500}>
          <LineChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
            <CartesianGrid strokeDasharray="3 3" stroke="#e2e8f0" />
            <XAxis 
              dataKey="year" 
              label={{ value: 'Year', position: 'insideBottom', offset: -5 }}
              stroke="#64748b"
            />
            <YAxis 
              domain={[1000, 1700]}
              label={{ value: 'Supply (Millions of KOR)', angle: -90, position: 'insideLeft' }}
              stroke="#64748b"
            />
            <Tooltip 
              contentStyle={{ 
                backgroundColor: '#f8fafc', 
                border: '1px solid #cbd5e1',
                borderRadius: '8px'
              }}
              formatter={(value) => [`${value}M KOR`, '']}
            />
            <Legend 
              wrapperStyle={{ paddingTop: '20px' }}
              iconType="line"
            />
            <defs>
              <linearGradient id="areaGradient" x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="#9ca3af" stopOpacity={0.4} />
                <stop offset="100%" stopColor="#9ca3af" stopOpacity={0.2} />
              </linearGradient>
            </defs>
            <Area
              type="monotone"
              dataKey="5% Growth"
              stroke="none"
              fill="url(#areaGradient)"
            />
            <Area
              type="monotone"
              dataKey="2% Growth"
              stroke="none"
              fill="#ffffff"
              fillOpacity={1}
            />
            <Line 
              type="monotone" 
              dataKey="2% Growth" 
              stroke="#3b82f6" 
              strokeWidth={2}
              dot={false}
            />
            <Line 
              type="monotone" 
              dataKey="5% Growth" 
              stroke="#ef4444" 
              strokeWidth={2}
              dot={false}
            />
          </LineChart>
        </ResponsiveContainer>

        <div className="mt-6 grid grid-cols-2 gap-4">
          <div className="bg-blue-50 p-4 rounded-lg">
            <div className="text-blue-600 text-sm font-semibold">2% Growth</div>
            <div className="text-2xl font-bold text-blue-900">
              {data[10]['2% Growth']}M
            </div>
            <div className="text-xs text-blue-600">After 10 years</div>
          </div>
          <div className="bg-red-50 p-4 rounded-lg">
            <div className="text-red-600 text-sm font-semibold">5% Growth</div>
            <div className="text-2xl font-bold text-red-900">
              {data[10]['5% Growth']}M
            </div>
            <div className="text-xs text-red-600">After 10 years</div>
          </div>
        </div>

        <p className="text-sm text-slate-500 mt-4 text-center">
          Starting supply: {initialSupply} million KOR tokens
        </p>
      </div>
    </div>
  );
};

export default KORSupplyGrowth;