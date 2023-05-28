import axios from "axios";
import React from "react"
import { DATABASE_URL } from "../consts";
import ReactECharts from 'echarts-for-react';

const query_template = `
select count(*) as add_count, commit_time 
from records 
where repo_name = 'REPO_NAME' and operation = 'add' 
group by commit_time
order by commit_time;
select count(*) as remove_count, commit_time 
from records 
where repo_name = 'REPO_NAME' and operation = 'remove' 
group by commit_time
order by commit_time;
`

function convert_timestamp(timestamp: string): string {
    let date = new Date(parseInt(timestamp) * 1000);
    return date.toLocaleString();
}

// Line chart that shows the count of remove and add operations over time
export default function OperationHistory(props: { repo_name: string }) {
    const [chart_data, set_chart_data] = React.useState<{
        curr_repo: string, data: Map<string, { add: number, remove: number }>
    }>({ curr_repo: "", data: new Map() })

    let query = query_template.replaceAll("REPO_NAME", props.repo_name);
    let data = axios.post(`${DATABASE_URL}/v1/sql?db=public`,
        { sql: query },
        {
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
                "Access-Control-Allow-Origin": "*"
            }
        }
    ).then(function (response) {
        let data = new Map();
        // fill add query result
        for (let i = 0; i < response.data.output[0].records.rows.length; i++) {
            let curr = response.data.output[0].records.rows[i];
            let add = parseInt(curr[0]);
            let time = curr[1];

            if (data.has(time)) {
                data.get(time).add += add;
            } else {
                data.set(time, { add: add, remove: 0 });
            }
        }
        // fill remove query result
        for (let i = 0; i < response.data.output[1].records.rows.length; i++) {
            let curr = response.data.output[1].records.rows[i];
            let remove = parseInt(curr[0]);
            let time = curr[1];

            if (data.has(time)) {
                data.get(time).remove += remove;
            } else {
                data.set(time, { add: 0, remove: remove });
            }
        }

        if (chart_data.curr_repo !== props.repo_name) {
            set_chart_data({ curr_repo: props.repo_name, data: data })
        }
    });

    const chart_options = () => {
        let time: string[] = [];
        let add: number[] = [];
        let remove: number[] = [];

        let order_list: { time: string, add: number, remove: number }[] = [];
        chart_data.data.forEach((value, key) => {
            order_list.push({ time: key, add: value.add, remove: -value.remove });
        })
        order_list.sort((a, b) => {
            return a.time > b.time ? 1 : -1;
        })
        order_list.forEach((value) => {
            time.push(convert_timestamp(value.time));
            add.push(value.add);
            remove.push(value.remove);
        })

        return {
            title: {
                text: 'TODO Operation History',
                left: 'center',
            },
            tooltip: {
                trigger: 'axis',
                axisPointer: {
                    type: 'shadow'
                }
            },
            grid: {
                left: '3%',
                right: '4%',
                bottom: '3%',
                containLabel: true,
            },
            xAxis: [
                {
                    type: 'category',
                    data: time
                }
            ],
            yAxis: [
                {
                    type: 'value',
                }
            ],
            series: [
                {
                    name: 'Add',
                    type: 'line',
                    emphasis: {
                        focus: 'series'
                    },
                    data: add
                },
                {
                    name: 'Remove',
                    type: 'line',
                    emphasis: {
                        focus: 'series'
                    },
                    data: remove
                }
            ]
        }
    }

    return (<>
        <ReactECharts option={chart_options()} style={{ height: '500px' }} />
    </>)
}
