import React, { Component, PropTypes } from 'react';

import { TextField } from 'material-ui';
import { CardText } from 'material-ui/Card';

import { FundAccount, Transfer } from '../../dialogs';

import Balances from '../../Balances';
import Container from '../../Container';
import Form, { FormWrap } from '../../Form';
import IdentityIcon from '../../IdentityIcon';

import Actions from './actions';

export default class Account extends Component {
  static contextTypes = {
    api: React.PropTypes.object
  }

  static propTypes = {
    params: PropTypes.object
  }

  state = {
    name: 'Unnamed',
    fundDialog: false,
    transferDialog: false
  }

  componentWillMount () {
    this.retrieveMeta();
  }

  render () {
    const address = this.props.params.address;

    return (
      <div>
        <FundAccount
          address={ address }
          onClose={ this.onFundAccountClose }
          visible={ this.state.fundDialog } />
        <Transfer
          address={ address }
          onClose={ this.onTransferClose }
          visible={ this.state.transferDialog } />
        <Actions
          onFundAccount={ this.onFundAccountClick }
          onTransfer={ this.onTransferClick } />
        <Container>
          <IdentityIcon
            address={ address } />
          <CardText>
            <Form>
              <FormWrap>
                <TextField
                  autoComplete='off'
                  floatingLabelText='account name'
                  fullWidth
                  hintText='a descriptive name for the account'
                  value={ this.state.name }
                  onChange={ this.onEditName } />
              </FormWrap>
              <FormWrap>
                <TextField
                  autoComplete='off'
                  disabled
                  floatingLabelText='account address'
                  fullWidth
                  hintText='the account network address'
                  value={ address } />
              </FormWrap>
            </Form>
            <Balances
              address={ address } />
          </CardText>
        </Container>
      </div>
    );
  }

  onFundAccountClick = () => {
    this.setState({ fundDialog: !this.state.fundDialog });
  }

  onFundAccountClose = () => {
    this.onFundAccountClick();
  }

  onTransferClick = () => {
    this.setState({ transferDialog: !this.state.transferDialog });
  }

  onTransferClose = () => {
    this.onTransferClick();
  }

  onEditName = (event) => {
    const api = this.context.api;
    const name = event.target.value;

    this.setState({
      name: name
    }, () => {
      api.personal.setAccountName(this.props.params.address, name);
    });
  }

  retrieveMeta () {
    this.context.api.personal
      .accountsInfo()
      .then((infos) => {
        const info = infos[this.props.params.address];
        this.setState({
          name: info.name,
          uuid: info.uuid,
          meta: info.meta
        });
      });
  }
}