package net.mullvad.mullvadvpn.service.tunnelstate

import android.content.Context
import android.content.SharedPreferences.OnSharedPreferenceChangeListener
import java.net.InetSocketAddress
import net.mullvad.mullvadvpn.model.TunnelState
import net.mullvad.talpid.net.Endpoint
import net.mullvad.talpid.net.TransportProtocol
import net.mullvad.talpid.net.TunnelEndpoint
import net.mullvad.talpid.tunnel.ActionAfterDisconnect
import net.mullvad.talpid.tunnel.ErrorState
import net.mullvad.talpid.tunnel.ErrorStateCause

private const val SHARED_PREFERENCES = "tunnel_state"
private const val KEY_TUNNEL_STATE = "tunnel_state"

private const val DISCONNECTED = "disconnected"
private const val CONNECTING = "connecting"
private const val CONNECTED = "connected"
private const val RECONNECTING = "reconnecting"
private const val DISCONNECTING = "disconnecting"
private const val BLOCKING = "blocking"
private const val ERROR = "error"

internal class Persistence(context: Context) {
    // TODO: Consider replacing the strings with an actual `Bundle` with the `TunnelState` object
    private val dummyTunnelEndpoint = TunnelEndpoint(Endpoint(
        InetSocketAddress.createUnresolved("dummy", 53),
        TransportProtocol.Tcp
    ))

    val sharedPreferences =
        context.getSharedPreferences(SHARED_PREFERENCES, Context.MODE_PRIVATE)

    var state
        get() = loadState()
        set(value) {
            persistState(value)
        }

    var listener: OnSharedPreferenceChangeListener? = null
        set(value) {
            if (value != field) {
                if (field != null) {
                    sharedPreferences.unregisterOnSharedPreferenceChangeListener(field)
                }

                if (value != null) {
                    sharedPreferences.registerOnSharedPreferenceChangeListener(value)
                }

                field = value
            }
        }

    private fun loadState(): TunnelState {
        val description = sharedPreferences.getString(KEY_TUNNEL_STATE, DISCONNECTED)

        return mapDescriptionToState(description)
    }

    private fun persistState(state: TunnelState) {
        sharedPreferences
            .edit()
            .putString(KEY_TUNNEL_STATE, mapStateToDescription(state))
            .commit()
    }

    private fun mapStateToDescription(state: TunnelState): String = when (state) {
        is TunnelState.Disconnected -> DISCONNECTED
        is TunnelState.Connecting -> CONNECTING
        is TunnelState.Connected -> CONNECTED
        is TunnelState.Disconnecting -> {
            if (state.actionAfterDisconnect == ActionAfterDisconnect.Reconnect) {
                RECONNECTING
            } else {
                DISCONNECTING
            }
        }
        is TunnelState.Error -> {
            if (state.errorState.isBlocking) {
                BLOCKING
            } else {
                ERROR
            }
        }
    }

    private fun mapDescriptionToState(description: String): TunnelState = when (description) {
        DISCONNECTED -> TunnelState.Disconnected()
        CONNECTING -> TunnelState.Connecting(null, null)
        CONNECTED -> TunnelState.Connected(dummyTunnelEndpoint, null)
        RECONNECTING -> TunnelState.Disconnecting(ActionAfterDisconnect.Reconnect)
        DISCONNECTING -> TunnelState.Disconnecting(ActionAfterDisconnect.Nothing)
        BLOCKING -> TunnelState.Error(ErrorState(ErrorStateCause.StartTunnelError(), true))
        ERROR -> TunnelState.Error(ErrorState(ErrorStateCause.SetFirewallPolicyError(), false))
        else -> TunnelState.Error(ErrorState(ErrorStateCause.SetFirewallPolicyError(), false))
    }
}
