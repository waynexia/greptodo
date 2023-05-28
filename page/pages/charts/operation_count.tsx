import axios from "axios";
import React from "react";
import { DATABASE_URL } from "../consts";
import ReactECharts from 'echarts-for-react';

const query_template = `
SELECT operation, count(*) as count from records where repo_name = 'REPO_NAME' group by operation order by operation;
`

// Pie chart that shows the count of remove and add operations
export default function OperationCount(props: { repo_name: string }) {
    const [chart_data, set_chart_data] = React.useState<{ curr_repo: string, add: number, remove: number }>({ curr_repo: "", add: 0, remove: 0 })

    let query = query_template.replace("REPO_NAME", props.repo_name);
    let data = axios.post(`${DATABASE_URL}/v1/sql?db=public`,
        { sql: query },
        {
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
                "Access-Control-Allow-Origin": "*"
            }
        }
    ).then(function (response) {
        let add = response.data.output[0].records.rows[0][1];
        let remove = response.data.output[0].records.rows[1][1];

        if (chart_data.curr_repo !== props.repo_name) {
            set_chart_data({ curr_repo: props.repo_name, add: add, remove: remove })
        }
    });

    const chart_options = () => {
        return {
            title: {
                text: 'TODO Operation Count',
                left: 'center',
            },
            tooltip: {},
            series: [
                {
                    type: 'pie',
                    data: [
                        {
                            value: chart_data.add,
                            name: 'Add'
                        },
                        {
                            value: chart_data.remove,
                            name: 'Remove'
                        },
                    ],
                    radius: ['40%', '70%'],
                    label: {
                        show: true,
                    },
                }
            ]
        }
    }

    return (<>
        <ReactECharts option={chart_options()} />
    </>)
}
