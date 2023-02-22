//REF https://stackblitz.com/edit/react-a7voza?file=index.js

import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';
import { createRoot } from 'react-dom/client';
import { Column } from '@ant-design/plots';

const StockChange = () => {
  const data = [
    { t: '9:30', v: -0.03 },
    { t: '9:45', v: -0.6 },
    { t: '10:00', v: 0.11 },
    { t: '10:15', v: 0.21 },
    { t: '10:30', v: 0.13 },
    { t: '10:45', v: 0.14 },
    { t: '11:00', v: 0.51 },
    { t: '11:15', v: 0.16 },
    { t: '11:30', v: 0.19 },
    { t: '1:00', v: -0.31 },
    { t: '1:15', v: -0.14 },
    { t: '1:35', v: 0.16 },
    { t: '1:45', v: -0.71 },
    { t: '2:00', v: -0.18 },
    { t: '2:15', v: -0.15 },
    { t: '2:30', v: -0.11 },
    { t: '2:45', v: 0.1 },
    { t: '3:00', v: 0.2 },
  ];

  const paletteSemanticRed = '#F4664A';
  const brandColor = '#5B8FF9';
  const config = {
    data,
    xField: 't',
    yField: 'v',
    seriesField: 'v',
    color: ({ v }) => {
      if (v > 0) {
        return paletteSemanticRed;
      }

      return brandColor;
    },
    label: {
      content: (originData) => {
        const val = parseFloat(originData.v);
        if (val < 0.05) {
          return (val * 100).toFixed(1) + '%';
        }
      },
      offset: 10,
    },
    legend: false,
    xAxis: {
      label: {
        autoHide: true,
        autoRotate: false,
      },
    },
  };
  return <Column {...config} />;
};

var container = document.getElementById('container');
createRoot(container).render(<StockChange />);


const table = 
	<table cellpadding="0" border="0" cellspacing="0">
		<thead>
			<tr>
				<th class="t_left">名称</th>
				<th><a href="javascript:void(0)" sort="symbol" class="sort --">代码</a></th>
				<td><a href="javascript:void(0)" sort="price" class="sort --">最新价</a></td>
				<td><a href="javascript:void(0)" sort="diff" class="sort --">涨跌额</a></td>
				<td><a href="javascript:void(0)" sort="chg" class="sort --">涨跌幅</a></td>
				<td><a href="javascript:void(0)" sort="volume" class="sort --">成交量</a></td>
				<td><a href="javascript:void(0)" sort="mktcap" class="sort --">市值</a></td>
				<td><a href="javascript:void(0)" sort="pe" class="sort --">市盈率</a></td>
				<th><a href="javascript:void(0)" sort="category" class="sort --">所属行业</a></th>
				<th><a href="javascript:void(0)" sort="market" class="sort --">上市地</a></th>
			</tr>
		</thead>
		<tbody>
			<tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/WMT.html">沃尔玛公司</a></th>
				<td>WMT</td>
				<td>147.33</td>
				<td class="up">0.89</td>
				<td class="up">0.61%</td>
				<td>1813.13万</td>
				<td>3973.20亿</td>
				<td>34.34</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c52" target="_blank">零售</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/PG.html">宝洁公司</a></th>
				<td>PG</td>
				<td>139.91</td>
				<td class="down">-0.10</td>
				<td class="down">-0.07%</td>
				<td>689.95万</td>
				<td>3300.68亿</td>
				<td>23.83</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c54" target="_blank">化妆品/护理</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/TRV.html">旅行者财产险集团</a></th>
				<td>TRV</td>
				<td>185.25</td>
				<td class="down">-0.50</td>
				<td class="down">-0.27%</td>
				<td>140.26万</td>
				<td>429.95亿</td>
				<td>15.55</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c25" target="_blank">保险</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/MRK.html">默沙东集团</a></th>
				<td>MRK</td>
				<td>109.07</td>
				<td class="down">-0.45</td>
				<td class="down">-0.41%</td>
				<td>674.14万</td>
				<td>2765.36亿</td>
				<td>19.00</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c10" target="_blank">制药</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/KO.html">可口可乐公司</a></th>
				<td>KO</td>
				<td>59.80</td>
				<td class="down">-0.32</td>
				<td class="down">-0.53%</td>
				<td>1427.81万</td>
				<td>2588.14亿</td>
				<td>27.06</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c74" target="_blank">饮料</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/MCD.html">麦当劳公司</a></th>
				<td>MCD</td>
				<td>268.55</td>
				<td class="down">-1.44</td>
				<td class="down">-0.53%</td>
				<td>282.42万</td>
				<td>1966.92亿</td>
				<td>46.38</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c52" target="_blank">零售</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/AMGN.html">安进公司</a></th>
				<td>AMGN</td>
				<td>238.24</td>
				<td class="down">-2.29</td>
				<td class="down">-0.95%</td>
				<td>252.43万</td>
				<td>1272.15亿</td>
				<td>19.56</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c39" target="_blank">生物技术</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mO" target="_blank">纳斯达克</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/CVX.html">雪佛龙公司</a></th>
				<td>CVX</td>
				<td>161.00</td>
				<td class="down">-1.85</td>
				<td class="down">-1.14%</td>
				<td>725.97万</td>
				<td>3113.16亿</td>
				<td>8.78</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c76" target="_blank">油气</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/V.html">维萨卡公司</a></th>
				<td>V</td>
				<td>220.62</td>
				<td class="down">-2.94</td>
				<td class="down">-1.32%</td>
				<td>727.59万</td>
				<td>4537.13亿</td>
				<td>30.64</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c16" target="_blank">商业服务</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr><tr>
				<th class="t_left"><a href="http://stock.finance.sina.com.cn/usstock/quotes/JNJ.html">强生公司</a></th>
				<td>JNJ</td>
				<td>158.00</td>
				<td class="down">-2.39</td>
				<td class="down">-1.49%</td>
				<td>942.37万</td>
				<td>4114.77亿</td>
				<td>23.13</td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#c10" target="_blank">制药</a></td>
				<td><a href="http://finance.sina.com.cn/stock/usstock/sector.shtml#mN" target="_blank">纽约交易所</a></td>
			</tr>
		</tbody>
	</table>


import React, { useState, useEffect } from 'react';
import ReactDOM from 'react-dom';
import { Column } from '@ant-design/plots';

const DemoColumn = () => {
  const data = [
    {
      type: '1-3秒',
      value: 0.16,
    },
    {
      type: '4-10秒',
      value: 0.125,
    },
    {
      type: '11-30秒',
      value: 0.24,
    },
    {
      type: '31-60秒',
      value: 0.19,
    },
    {
      type: '1-3分',
      value: 0.22,
    },
    {
      type: '3-10分',
      value: 0.05,
    },
    {
      type: '10-30分',
      value: 0.01,
    },
    {
      type: '30+分',
      value: 0.015,
    },
  ];
  const paletteSemanticRed = '#F4664A';
  const brandColor = '#5B8FF9';
  const config = {
    data,
    xField: 'type',
    yField: 'value',
    seriesField: '',
    color: ({ type }) => {
      if (type === '10-30分' || type === '30+分') {
        return paletteSemanticRed;
      }

      return brandColor;
    },
    label: {
      content: (originData) => {
        const val = parseFloat(originData.value);

        if (val < 0.05) {
          return (val * 100).toFixed(1) + '%';
        }
      },
      offset: 10,
    },
    legend: false,
    xAxis: {
      label: {
        autoHide: true,
        autoRotate: false,
      },
    },
  };
  return <Column {...config} />;
};

ReactDOM.render(<DemoColumn />, document.getElementById('container'));
