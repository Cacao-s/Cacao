package com.ruru.cacao.data

import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.db.Member
import com.ruru.cacao.security.PinHasher
import com.ruru.cacao.security.RecoveryCode
import java.time.Clock
import java.time.Instant
import java.util.UUID

enum class Role(val code: String) { PARENT("parent"), ELDER("elder"), CHILD("child") }

val Member.roleEnum: Role get() = Role.entries.first { it.code == role }

class MemberRepository(
    private val db: CacaoDatabase,
    private val clock: Clock = Clock.systemUTC(),
) {
    private val queries get() = db.memberQueries
    private val palette = listOf("peach", "mint", "lilac", "sky", "butter", "rose")

    fun hasFamily(): Boolean = queries.countActive().executeAsOne() > 0

    fun members(): List<Member> = queries.active().executeAsList()

    fun member(id: String): Member? = queries.byId(id).executeAsOneOrNull()

    /** 建立父母並回傳救援碼（只顯示這一次） */
    fun createParent(name: String, pin: String): Pair<String, String> {
        val code = RecoveryCode.generate()
        val id = insert(Role.PARENT, name, null, PinHasher.hash(pin), PinHasher.hash(RecoveryCode.normalize(code)))
        return id to code
    }

    fun createChild(name: String, birthday: String): String = insert(Role.CHILD, name, birthday, null, null)

    fun createElder(name: String): String = insert(Role.ELDER, name, null, null, null)

    fun verifyPin(memberId: String, pin: String): Boolean {
        val hash = member(memberId)?.pin_hash ?: return false
        return PinHasher.verify(pin, hash)
    }

    /** 孩子模式按鎖頭：任一位父母的 PIN 都可以，回傳是哪一位 */
    fun parentForPin(pin: String): Member? =
        members().filter { it.roleEnum == Role.PARENT }.firstOrNull { verifyPin(it.id, pin) }

    fun resetPinByOtherParent(targetId: String, helperId: String, helperPin: String, newPin: String): String? {
        if (targetId == helperId || !verifyPin(helperId, helperPin)) return null
        return replacePin(targetId, newPin)
    }

    fun resetPinByRecovery(targetId: String, recoveryInput: String, newPin: String): String? {
        val hash = member(targetId)?.recovery_hash ?: return null
        if (!PinHasher.verify(RecoveryCode.normalize(recoveryInput), hash)) return null
        return replacePin(targetId, newPin)
    }

    fun verifyChildPassword(childId: String, password: String): Boolean {
        val hash = member(childId)?.password_hash ?: return true
        return PinHasher.verify(password, hash)
    }

    fun setChildPassword(childId: String, password: String?) {
        queries.updatePassword(password?.let(PinHasher::hash), now(), childId)
    }

    /** 換 PIN 時救援碼也換新的（用過就作廢） */
    private fun replacePin(memberId: String, newPin: String): String {
        val code = RecoveryCode.generate()
        queries.updatePin(PinHasher.hash(newPin), PinHasher.hash(RecoveryCode.normalize(code)), now(), memberId)
        return code
    }

    private fun insert(role: Role, name: String, birthday: String?, pinHash: String?, recoveryHash: String?): String {
        val id = UUID.randomUUID().toString()
        val order = members().size.toLong()
        val time = now()
        queries.insert(id, role.code, name.trim(), birthday, palette[(order % palette.size).toInt()], pinHash, recoveryHash, null, order, time, time)
        return id
    }

    private fun now(): String = Instant.now(clock).toString()
}
