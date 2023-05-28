import axios from "axios";
import React from "react";
import { DATABASE_URL } from "../consts";
import ReactECharts from 'echarts-for-react';

const query_template = `
select count(*) as add_count, author_name
from records 
where repo_name = 'REPO_NAME' and operation = 'add'
group by author_name
order by add_count desc;
select count(*) as remove_count, author_name
from records 
where repo_name = 'REPO_NAME' and operation = 'remove'
group by author_name
order by remove_count desc;
`

// Bar chart that shows the author's TODO items
export default function AuthorRank(props: { repo_name: string }) {
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
            let author = curr[1];

            if (data.has(author)) {
                data.get(author).add += add;
            } else {
                data.set(author, { add: add, remove: 0 });
            }
        }
        // fill remove query result
        for (let i = 0; i < response.data.output[1].records.rows.length; i++) {
            let curr = response.data.output[1].records.rows[i];
            let remove = parseInt(curr[0]);
            let author = curr[1];

            if (data.has(author)) {
                data.get(author).remove += remove;
            } else {
                data.set(author, { add: 0, remove: remove });
            }
        }

        if (chart_data.curr_repo !== props.repo_name) {
            set_chart_data({ curr_repo: props.repo_name, data: data })
        }
    });

    const chart_options = () => {
        let author: string[] = [];
        let add: number[] = [];
        let remove: number[] = [];
        let total: number[] = [];

        let order_list: { author: string, add: number, remove: number, total: number }[] = [];
        chart_data.data.forEach((value, key) => {
            order_list.push({ author: key, add: value.add, remove: -value.remove, total: value.add - value.remove });
        })
        order_list.sort((a, b) => {
            return a.total > b.total ? 1 : -1;
        })
        order_list.forEach((value) => {
            author.push(value.author);
            add.push(value.add);
            remove.push(value.remove);
            total.push(value.total);
        })

        return {
            title: {
                text: 'TODO Author Rank',
                left: 'center',
            },
            tooltip: {
                trigger: 'axis',
                axisPointer: {
                    type: 'shadow'
                }
            },
            grid: {
                left: '10%',
                right: '4%',
                bottom: '3%',
                containLabel: true,
            },
            xAxis: [
                {
                    type: 'value'
                }
            ],
            yAxis: [
                {
                    type: 'category',
                    axisTick: {
                        show: false
                    },
                    data: author
                }
            ],
            series: [
                {
                    name: 'Total',
                    type: 'bar',
                    label: {
                        show: true,
                        position: 'right'
                    },
                    emphasis: {
                        focus: 'series'
                    },
                    data: total
                },
                {
                    name: 'Add',
                    type: 'bar',
                    stack: 'Total',
                    emphasis: {
                        focus: 'series'
                    },
                    data: add
                },
                {
                    name: 'remove',
                    type: 'bar',
                    stack: 'Total',
                    emphasis: {
                        focus: 'series'
                    },
                    data: remove
                }
            ]
        }
    }

    return (<>
        <ReactECharts option={chart_options()} style={{ height: '800px' }} />
    </>)
}
