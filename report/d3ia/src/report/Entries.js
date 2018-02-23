import React, { Component } from 'react'

import { Row, Table } from 'react-bootstrap';

export default class Entries extends Component {
    render() {
        const rows = () => {
          return this.props.data.map((entry, index) => {
              return <tr key={index}>
                <td>{index}</td>
                <td>{entry.product}</td>
                <td>{entry.price}</td>
                <td>{entry.time}</td>
                <td>{entry.category}</td>
              </tr>
          })
        }

        return <Row>
            <Table striped bordered condensed hover>
              <thead>
                <tr>
                  <th>#</th>
                  <th>Товар</th>
                  <th>Цена</th>
                  <th>Дата</th>
                  <th>Категория</th>
                </tr>
              </thead>
              <tbody>
                {rows()}
              </tbody>
            </Table>
          </Row>;
    }
}
