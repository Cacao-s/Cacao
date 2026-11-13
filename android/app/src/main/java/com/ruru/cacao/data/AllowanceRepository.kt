package com.ruru.cacao.data

import com.ruru.cacao.db.Allowance_schedule
import com.ruru.cacao.db.CacaoDatabase
import com.ruru.cacao.domain.AllowanceDates
import com.ruru.cacao.domain.Period
import java.time.Clock
import java.time.Instant
import java.time.LocalDate
import java.util.UUID

data class ScheduleInput(
    val sourceId: String,
    val amount: Long,
    val period: Period,
    val weekday: Int?,
    val monthDay: Int?,
    val startDate: LocalDate? = null,
)

val Allowance_schedule.periodEnum: Period get() = Period.entries.first { it.code == period }

class AllowanceRepository(
    private val db: CacaoDatabase,
    private val clock: Clock = Clock.systemDefaultZone(),
) {
    private val queries get() = db.allowanceQueries

    fun schedules(childId: String): List<Allowance_schedule> = queries.forChild(childId).executeAsList()

    /** 開始日期在過去就從那天補發到今天；前後限一年內，避免打錯年份一次補發太多 */
    fun create(childId: String, input: ScheduleInput): Boolean {
        val start = input.startDate ?: today()
        if (!valid(input) || !validStart(start)) return false
        val time = now()
        val id = UUID.randomUUID().toString()
        queries.insert(id, childId, input.sourceId, input.amount, input.period.code, input.weekday?.toLong(), input.monthDay?.toLong(), start.toString(), time, time)
        backfill(id)
        return true
    }

    /** 修改前先補記，過去的期數用舊金額、之後才用新設定（「之後照新金額發」） */
    fun update(scheduleId: String, input: ScheduleInput): Boolean {
        if (!valid(input)) return false
        backfill(scheduleId)
        queries.update(input.sourceId, input.amount, input.period.code, input.weekday?.toLong(), input.monthDay?.toLong(), nextStart(scheduleId).toString(), now(), scheduleId)
        return true
    }

    /** 暫停前先補記；恢復時從今天起算，暫停期間不補發 */
    fun setPaused(scheduleId: String, paused: Boolean) {
        val schedule = queries.byId(scheduleId).executeAsOneOrNull() ?: return
        if (paused) {
            backfill(scheduleId)
            queries.setPaused(1, schedule.active_from, now(), scheduleId)
        } else {
            queries.setPaused(0, today().toString(), now(), scheduleId)
            backfill(scheduleId)
        }
    }

    fun delete(scheduleId: String) {
        backfill(scheduleId)
        val time = now()
        queries.softDelete(time, time, scheduleId)
    }

    /** 開 App 時呼叫：所有未暫停的排程補記到今天；回傳補了幾筆 */
    fun backfillAll(): Int = queries.active().executeAsList().sumOf { backfill(it.id) }

    private fun backfill(scheduleId: String): Int {
        val s = queries.byId(scheduleId).executeAsOneOrNull() ?: return 0
        if (s.paused == 1L || s.deleted_at != null) return 0
        val last = queries.lastDue(scheduleId).executeAsOne().last?.let(LocalDate::parse)
        val from = last?.plusDays(1) ?: LocalDate.parse(s.active_from)
        val dates = AllowanceDates.due(s.periodEnum, s.weekday?.toInt(), s.month_day?.toInt(), LocalDate.parse(s.active_from), from, today())
        val time = now()
        db.transaction {
            dates.forEach { due ->
                val id = UUID.randomUUID().toString()
                db.ledgerQueries.insertAllowanceEntry(id, id, s.child_id, s.amount, due.toString(), s.source_id, s.id, due.toString(), time)
            }
        }
        return dates.size
    }

    /** 修改後從下一天起用新設定，避免同一天用新規則重發 */
    private fun nextStart(scheduleId: String): LocalDate {
        val last = queries.lastDue(scheduleId).executeAsOne().last?.let(LocalDate::parse)
        return maxOf(today(), last?.plusDays(1) ?: today())
    }

    private fun valid(input: ScheduleInput): Boolean = input.amount > 0 && when (input.period) {
        Period.DAILY -> true
        Period.WEEKLY, Period.BIWEEKLY -> input.weekday in 1..7
        Period.MONTHLY -> input.monthDay in 1..31
    }

    fun validStart(start: LocalDate): Boolean =
        !start.isBefore(today().minusYears(1)) && !start.isAfter(today().plusYears(1))

    private fun today(): LocalDate = LocalDate.now(clock)

    private fun now(): String = Instant.now(clock).toString()
}
